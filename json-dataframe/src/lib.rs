
use fluvio_smartmodule::dataplane::smartmodule::{SmartModuleExtraParams};



use fluvio_smartmodule::{smartmodule, Result, Record, RecordData};

#[smartmodule(map)]
pub fn map(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    let key = record.key.clone();

    let string = std::str::from_utf8(record.value.as_ref())?;
    let int = string.parse::<i32>()?;
    let value = (int * 2).to_string();

    Ok((key, value.into()))
}

#[smartmodule(init)]
fn init(_params: SmartModuleExtraParams) -> Result<()> {
    // You can refer to the example SmartModules in Fluvio's GitHub Repository
    // https://github.com/infinyon/fluvio/tree/master/smartmodule
    todo!("Provide initialization logic for your SmartModule")
}


