/*
    Create a struct that holds info about a gun(gun type, recoil time, magazine size, extra)
    where gun type and extra are enums and extra contains silencer, scope, extended mags nad None.
    Based on user input change the value of extra (may cause change in recoil time and magazine size).
 */

pub mod better_gun_program {
    use std::io;
    use std::time::Duration;

    #[derive(Debug, Clone)]
    struct Gun {
        recoil_time: Duration,
        gun_type: GunType,
        magazine_size: i8,
        extra: Extra
    }

    #[derive(Debug, Clone)]
    enum GunType {
        HandGun,
        MachineGun,
        ShotGun,
        LongGun
    }

    #[derive(Debug, Clone)]
    enum Extra {
        Silencer,
        Scope,
        ExtendedMags,
        None
    }

    impl Gun {
        fn new(recoil_time: Duration, gun_type: GunType, magazine_size: i8, extra: Extra) -> Self {
            Gun {
                recoil_time,
                gun_type,
                magazine_size,
                extra
            }
        }

        fn apply_extra(&mut self, new_extra: Extra) {
            match new_extra {
                Extra::Silencer => {
                    self.recoil_time += Duration::from_secs(2);
                }
                Extra::Scope => {
                    // No effect on stats
                }
                Extra::ExtendedMags => {
                    self.magazine_size += 5;
                }
                Extra::None => {
                    // Reset to base values
                    match self.gun_type {
                        GunType::HandGun => {
                            self.recoil_time = Duration::from_secs(1);
                            self.magazine_size = 10;
                        }
                        GunType::MachineGun => {
                            self.recoil_time = Duration::from_secs(3);
                            self.magazine_size = 30;
                        }
                        GunType::ShotGun => {
                            self.recoil_time = Duration::from_secs(4);
                            self.magazine_size = 6;
                        }
                        GunType::LongGun => {
                            self.recoil_time = Duration::from_secs(2);
                            self.magazine_size = 5;
                        }
                    }
                }
            }
            self.extra = new_extra;
        }

        fn display(&self) {
            println!("\nGun Stats:");
            println!("Type: {:?}", self.gun_type);
            println!("Recoil Time: {:?}", self.recoil_time);
            println!("Magazine Size: {}", self.magazine_size);
            println!("Attachment: {:?}", self.extra);
        }
    }

    fn get_user_choice() -> Extra {
        loop {
            println!("\nChoose an attachment:");
            println!("1. Silencer (+2s recoil)");
            println!("2. Scope (no stat change)");
            println!("3. Extended Mags (+5 capacity)");
            println!("4. None (reset to base stats)");
            println!("Enter your choice (1-4):");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");

            match input.trim() {
                "1" => return Extra::Silencer,
                "2" => return Extra::Scope,
                "3" => return Extra::ExtendedMags,
                "4" => return Extra::None,
                _ => println!("Invalid input, please try again"),
            }
        }
    }

    pub fn gun_main() {
        // Create a base gun
        let mut my_gun = Gun::new(
            Duration::from_secs(1),
            GunType::HandGun,
            10,
            Extra::None
        );

        loop {
            my_gun.display();

            let choice = get_user_choice();
            my_gun.apply_extra(choice);

            println!("\nAttachment applied!");
        }
    }
}