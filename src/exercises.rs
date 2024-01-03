pub mod exercises {
    pub enum Exercise {
        Quicktype,
        Copy,
    }

    impl Exercise {
        pub fn get_name(&self) -> String {
            match self {
                Exercise::Quicktype => String::from("quicktype"),
                Exercise::Copy => String::from("copy"),
            }
        }

        pub fn get_path(&self) -> String {
            let base: String = String::from("./exercises/");
            base + match self {
                Exercise::Copy => "copy",
                Exercise::Quicktype => "quicktype",
            }
        }

        pub fn start(&self) {
            match self {
                Exercise::Quicktype => println!("starting the quicktype exercise"),
                Exercise::Copy => println!("starting the copy exercise"),
            }
        }
    }
}
