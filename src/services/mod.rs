pub mod service {
    use bluer::{gatt::remote::Characteristic, Device, Result, Uuid};

    pub async fn enumerate(device: &Device) -> Result<Option<Characteristic>> {
        // Try to connect
        connect(device).await?;

        // Enumerate Device
        for service in device.services().await? {
            let uuid = service.uuid().await?;
            println!("    Service UUID: {}", &uuid);
            println!("    Service data: {:?}", service.all_properties().await?);
            for char in service.characteristics().await? {
                let uuid = char.uuid().await?;
                println!("    Characteristic UUID: {}", &uuid);
                println!(
                    "    Characteristic data: {:?}",
                    char.all_properties().await?
                );
            }
        }

        Ok(None)
    }

    pub async fn read_characteristic(device: &Device, char_uuid: Uuid) -> Result<()> {
        //Try to connect
        connect(device).await?;

        //Enumerate device
        for service in device.services().await? {
            for char in service.characteristics().await? {
                let uuid = char.uuid().await?;
                if uuid == char_uuid {
                    println!("    Characteristic UUID: {}", &uuid);
                    println!(
                        "    Characteristic data: {:?}",
                        char.all_properties().await?
                    );
                }
            }
        }
        Ok(())
    }

    pub async fn write_characteristic(
        device: &Device,
        char_uuid: Uuid,
        data: String,
    ) -> Result<()> {
        //Try to connect
        connect(device).await?;

        //Enumerate device
        for service in device.services().await? {
            for char in service.characteristics().await? {
                let uuid = char.uuid().await?;
                if uuid == char_uuid {
                    println!("    Characteristic UUID: {}", &uuid);
                    println!(
                        "    Characteristic data: {:?}",
                        char.all_properties().await?
                    );
                }
            }
        }
        Ok(())
    }

    pub async fn connect(device: &Device) -> Result<()> {
        if !device.is_connected().await.expect("Could not find device") {
            let mut retries = 2;
            loop {
                match device.connect().await {
                    Ok(()) => break,
                    Err(_) if retries > 0 => {
                        retries -= 1;
                    }
                    Err(err) => return Err(err),
                }
            }
            println!("Connected!");
        } else {
            println!("Already Connected")
        }
        Ok(())
    }
}
