export const skills = [
  "Swift",
  "SwiftUI",
  "Laravel",
  "PHP",
  "TypeScript",
  "AWS",
  "MySQL",
  "Redis",
] as const;

export const projects = [
  {
    title: "Stacks",
    kind: "Social platform for iOS",
    description:
      "A media-first social product with a native SwiftUI experience and backend systems designed to keep uploads dependable.",
    stack: ["SwiftUI", "Laravel", "AWS S3", "MySQL"],
  },
  {
    title: "APIClient",
    kind: "Swift networking library",
    description:
      "A type-safe HTTP layer that keeps requests, responses, and multipart uploads easy to reason about across a growing iOS codebase.",
    stack: ["Swift", "Combine", "REST", "Protocols"],
  },
  {
    title: "ProcessQueue",
    kind: "Media processing pipeline",
    description:
      "A background processing system for validating uploads, converting image formats, and delivering ready-to-use media to S3.",
    stack: ["Laravel", "Queues", "AWS", "Redis"],
  },
] as const;
