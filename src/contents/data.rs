#[derive(Clone)]
pub struct ContentHighlightData {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub time: &'static str,
    pub description: Vec<&'static str>,
}

#[derive(Clone)]
pub struct CardData {
    pub heading: &'static str,
    pub description: &'static str,
    pub img: &'static str,
    pub link: &'static str,
}

#[derive(Clone)]
pub struct SkillData {
    pub skill_type: &'static str,
    pub skill_subtype: Vec<&'static str>,
}

pub fn get_career_data() -> Vec<ContentHighlightData> {
    // #1
    let srin = ContentHighlightData {
        title: "Software Engineer", 
        subtitle: "Samsung R&D Institute Indonesia", 
        time: "September 2023 - Present",
        description: vec![
            "Assigned as Lead Engineer that maintain .NET Framework integration on Tizen OS, resolving real-world market problems related to .NET application on Tizen TV.",
            "Collaborated in porting .NET Framework to Tizen TV on RISC-V Architecture, showcased on 2024 Samsung Developer Conference.",
            "Analyzed logs, source files, and core dumps to diagnose and resolve issues affecting .NET application on Tizen TV, including real-world market problems.",
            "Developed a Linux Daemon in C++ that manages TV workload after software update, improving responsiveness and performance of the TV.",
            "Prototyped a solution for running deep learning framework on Android for internal developers.",
        ]
    };

    // #2
    let xtremax = ContentHighlightData {
        title: "Software Developer (Backend)", 
        subtitle: "Xtremax Indonesia", 
        time: "February 2023 - August 2023",
        description: vec![
         "Performed maintenance, critical vulnerability fixes, user support, and development on a Singapore Government Website.",
         "Collaborated in deploying a custom feature for Sitecore valued in thousands of SGD.",
     ]
    };

    // #3
    let tokped = ContentHighlightData {
        title: "Software Engineer Intern (Backend) ", 
        subtitle: "Tokopedia", 
        time: "August 2021 - February 2022",
        description: vec![
         "Improved code reusability by refactor old code to “Clean Architecture” in voucher microservice.",
         "Improved features of voucher microservice, developing API with unit tests and documentations.",
     ]
    };

    vec![srin, xtremax, tokped]
}

pub fn get_edu_data() -> Vec<ContentHighlightData> {
    // #1
    let umn = ContentHighlightData {
        title: "Bachelor of Computer Engineering",
        subtitle: "Universitas Multimedia Nusantara",
        time: "August 2018 - July 2022",
        description: vec![
            "Grade: 3.99 / 4.00, Graduated with Distinction",
            "Best Graduate of Bachelor's Degree Graduation Batch XXIII, 2022",
        ],
    };

    vec![umn]
}

pub fn get_project_data() -> Vec<CardData> {
    // #0
    let website = CardData {
        heading:"Personal Website",
        description:"Web Development - Personal website built using Rust (Leptos) and TailwindCSS + daisyui. Techstack were choosen to challenge myself in learning new framework and designing a website",
        img:"images/website_personal.jpg",
        link:"https://github.com/ryukinlika/personal-website-csr",
    };

    // #1
    let nema = CardData {
        heading: "Nematode Classifier Image Recognition",
        description: "Web application developed using Python Flask library, allows user to upload an image and do inference
        using models combined with custom input and classification layers.",
        img:"images/nema_microscope.jpg",
        link:"https://github.com/ryukinlika/flaskclassifier"
    };

    // #2
    let trash = CardData {
        heading: "Deep Learning Based Trash Separator",
        description: "Web application and Backend developed with Golang to monitor trash bin status and detected items. Implement AI image recognition on Raspberry Pi using camera and deep learning using Python and TensorFlow Lite.", 
        img:"images/trash_plastic.jpg",
        link:"https://github.com/ryukinlika/trash-separator-api",
    };

    // #3
    let enhancer = CardData {
        heading: "Image Enhancer using Android Java",
        description: "Developed using Java language, together with Deep Learning libraries that increases the resolution of the input image using Generative AI.",    
        img:"images/enhancer_phone.jpg",
        link:"https://github.com/ryukinlika/resolution-enchancher-app",
    };

    // #4
    let inventory = CardData {
        heading: "Inventory Management System using Python",
        description: "Manages borrowing and returning process of an inventory database using RFID on Raspberry Pi Zero. Backend features basic authentication and CRUD operations on data stored in Google Sheets.", 
        img:"images/inventory_shelve.jpg",
        link:"https://github.com/ryukinlika/InventorySystem",
    };

    // #5
    let selfbal = CardData {
        heading: "Arduino-Based Self Balancing Robot",
        description: "Self balancing robot project using Arduino Nano Board with MPU6050 tracking (gyroscope & accelerometer) device", 
        img:"images/selfbal_arduino.jpg",
        link:"https://github.com/ryukinlika/selfbal",
    };

    // #6
    let micropractice = CardData {
        heading: "Microservice Architecture Practice",
        description: "Microservice architecture practice using Java Spring Boot framework",
        img: "images/microservice_code.jpg",
        link: "https://github.com/ryukinlika/microservice-practice",
    };

    vec![
        website,
        nema,
        trash,
        enhancer,
        inventory,
        selfbal,
        micropractice,
    ]
}

pub fn get_skill_data() -> Vec<SkillData> {
    // #1
    let general = SkillData {
        skill_type: "Programming Languages",
        skill_subtype: vec![
            "Golang",
            "C/C++",
            ".NET C#",
            "Python",
            "Java",
            "Tizen",
            "Spring Boot",
        ],
    };

    // #2
    let web = SkillData {
        skill_type: "Web Programming",
        skill_subtype: vec!["Javascript", "PHP", "HTML/CSS", "Node.js"],
    };

    // #3
    let database = SkillData {
        skill_type: "Database",
        skill_subtype: vec!["MySQL", "PostgreSQL", "Redis"],
    };

    // #4
    let others = SkillData {
        skill_type: "Others",
        skill_subtype: vec![
            "Linux",
            "Git",
            "Image Recognition",
            "Computer Vision",
            "CMake",
        ],
    };

    vec![general, web, database, others]
}
