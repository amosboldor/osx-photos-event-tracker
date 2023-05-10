use core::time;
use rusqlite;
use rusqlite::{params, backup, Connection, Result};


#[allow(dead_code)]
#[derive(Debug)]
struct Asset {
    date_created: String,
    kind: u8,
    latitude: f64,
    longitude: f64,
    directory: String,
    filename: String,
    saved_asset_type: u8,
    reverse_location_data: Vec<u8>,
    reverse_location_data_is_valid: bool,
    timezone_offset: f32,
    timezone_name: String,
    person_id: u16,
    fullname: String,
    source_height: u16,
    source_width: u16,
    centerx: f64,
    centery: f64,
}



fn get_in_memory_back_up(src: &mut Connection, dst: & Connection) -> Result<(), rusqlite::Error> {
    let backup = backup::Backup::new(&dst, src)?;
    backup.run_to_completion(
        17_000, 
        time::Duration::from_millis(1),
        None
    )?;
    Ok(())
}

fn retrieve_entries(photos_library_path: &str) -> Result<()> {
    let mut conn = Connection::open_in_memory()?;
    let dst = Connection::open(photos_library_path)?;
    get_in_memory_back_up(&mut conn, &dst)?;

    let mut stmt = conn.prepare(
        "SELECT
                datetime(ZDATECREATED, 'unixepoch', '31 years'),
                ZASSET.ZKIND,
                ZASSET.ZLATITUDE,
                ZASSET.ZLONGITUDE,
                ZASSET.ZDIRECTORY,
                ZASSET.ZFILENAME,
                ZASSET.ZSAVEDASSETTYPE,
                ZADDITIONALASSETATTRIBUTES.ZREVERSELOCATIONDATA,
                ZADDITIONALASSETATTRIBUTES.ZREVERSELOCATIONDATAISVALID,
                ZADDITIONALASSETATTRIBUTES.ZTIMEZONEOFFSET,
                ZADDITIONALASSETATTRIBUTES.ZTIMEZONENAME,
                ZDETECTEDFACE.ZPERSON,
                ZPERSON.ZFULLNAME,
                ZDETECTEDFACE.ZSOURCEHEIGHT,
                ZDETECTEDFACE.ZSOURCEWIDTH,
                ZDETECTEDFACE.ZCENTERX,
                ZDETECTEDFACE.ZCENTERY
         FROM ZDETECTEDFACE
         JOIN ZASSET ON ZASSET.Z_PK = ZDETECTEDFACE.ZASSET
         JOIN ZADDITIONALASSETATTRIBUTES ON ZADDITIONALASSETATTRIBUTES.ZASSET = ZASSET.Z_PK
         JOIN ZPERSON ON ZPERSON.Z_PK = ZDETECTEDFACE.ZPERSON
         WHERE ZADDITIONALASSETATTRIBUTES.ZREVERSELOCATIONDATA IS NOT NULL;"
    )?;

    let assets = stmt.query_map(params![], |row| {
        Ok(Asset {
            date_created: row.get(0)?,
            kind: row.get(1)?,
            latitude: row.get(2)?,
            longitude: row.get(3)?,
            directory: row.get(4)?,
            filename: row.get(5)?,
            saved_asset_type: row.get(6)?,
            reverse_location_data: row.get(7)?,
            reverse_location_data_is_valid: row.get(8)?,
            timezone_offset: row.get(9)?,
            timezone_name: row.get(10)?,
            person_id: row.get(11)?,
            fullname: row.get(12)?,
            source_height: row.get(13)?,
            source_width: row.get(14)?,
            centerx: row.get(15)?,
            centery: row.get(16)?,
        })
    })?;

    let assets: Result<Vec<_>> = assets.collect();
    println!("{:#?}", assets?);
    Ok(())
}
