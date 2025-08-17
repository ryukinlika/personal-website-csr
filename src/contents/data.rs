#[derive(Clone)]
pub struct ContentHighlightData {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub time: &'static str,
    pub description: Vec<&'static str>,
}

pub fn generate_career_data() -> Vec<ContentHighlightData> {
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

pub fn generate_edu_data() -> Vec<ContentHighlightData> {
    // #1
    let umn = ContentHighlightData { 
        title: "Bachelor of Computer Engineering", 
        subtitle: "Universitas Multimedia Nusantara", 
        time: "August 2018 - July 2022",
        description: vec![
            "Grade: 3.99 / 4.00, Graduated with Distinction",
            "Best Graduate of Bachelor's Degree Graduation Batch XXIII, 2022"
        ]
    }; 

    vec![umn]
}
