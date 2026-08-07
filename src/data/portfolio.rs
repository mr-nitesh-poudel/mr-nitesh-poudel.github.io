pub const CONTACT_EMAIL: &str = "contact@niteshpoudel.com";

pub const SKILLS: [&str; 8] = [
    "Swift",
    "SwiftUI",
    "Laravel",
    "PHP",
    "TypeScript",
    "AWS",
    "MySQL",
    "Redis",
];

pub struct Project {
    pub title: &'static str,
    pub kind: &'static str,
    pub description: &'static str,
    pub stack: &'static [&'static str],
}

pub const PROJECTS: [Project; 3] = [
    Project {
        title: "Stacks",
        kind: "Social platform for iOS",
        description: "A media-first social product with a native SwiftUI experience and backend systems designed to keep uploads dependable.",
        stack: &["SwiftUI", "Laravel", "AWS S3", "MySQL"],
    },
    Project {
        title: "APIClient",
        kind: "Swift networking library",
        description: "A type-safe HTTP layer that keeps requests, responses, and multipart uploads easy to reason about across a growing iOS codebase.",
        stack: &["Swift", "Combine", "REST", "Protocols"],
    },
    Project {
        title: "ProcessQueue",
        kind: "Media processing pipeline",
        description: "A background processing system for validating uploads, converting image formats, and delivering ready-to-use media to S3.",
        stack: &["Laravel", "Queues", "AWS", "Redis"],
    },
];
