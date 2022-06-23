use serde::Serialize;
use std::{collections::HashMap, fmt::Debug};

use crate::Compile;

pub fn compile(ctx: crate::type_check::BuildContext) -> Result<String, String> {
    Compose::compile(ctx)
        .and_then(|compose| serde_yaml::to_string(&compose).map_err(|e| format!("{:?}", e)))
}

#[derive(Debug, PartialEq, Serialize)]
struct Compose {
    version: &'static str,
    services: HashMap<String, Service>,
}

impl Compose {
    const VERSION: &'static str = "3";

    fn new(services: HashMap<String, Service>) -> Self {
        Self {
            version: Self::VERSION,
            services,
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
struct Service {
    image: &'static str,
    command: Command,
    healthcheck: HealthCheck,
}

impl Service {
    const GATES_IMG: &'static str = "ghcr.io/ncatelli/gates:main";

    fn new(command: Command, healthcheck: HealthCheck) -> Self {
        Self {
            image: Self::GATES_IMG,
            command,
            healthcheck,
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
#[repr(transparent)]
struct Command(String);

impl Command {
    fn new(gate: String, output_links: Vec<(String, char)>) -> Self {
        let links: Vec<_> = output_links
            .into_iter()
            .map(|(dest, input)| format!("http://{}:8080/input/{}", dest, input))
            .collect();

        if links.is_empty() {
            Command(format!("{} -listen-addr '0.0.0.0:8080'", gate,))
        } else {
            Command(format!(
                "{} -listen-addr '0.0.0.0:8080' -output-addrs '{}'",
                gate,
                links.join(",")
            ))
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
struct HealthCheck {
    test: Vec<String>,
}

impl HealthCheck {
    fn new<S: AsRef<str>>(port: S) -> Self {
        Self {
            test: format!("CMD curl -f http://127.0.0.1:{}/healthcheck", port.as_ref())
                .split(' ')
                .map(String::from)
                .collect(),
        }
    }
}

impl Default for HealthCheck {
    fn default() -> Self {
        Self::new("8080")
    }
}

impl crate::compiler::Compile for Compose {
    type Input = crate::type_check::BuildContext;

    fn compile(input: Self::Input) -> Result<Self, String> {
        let (mappings, gates, links) = input.into_raw_parts();
        let gate_services = mappings
            .keys()
            .zip(gates.iter())
            .zip(links.into_iter())
            .map(|((id, ty), links)| {
                let outbound_destinations = links
                    .into_iter()
                    .map(|link| (link.dest, link.input))
                    .collect();

                let cmd = Command::new(ty.as_str().to_string(), outbound_destinations);
                let svc = Service::new(cmd, HealthCheck::default());
                (id, svc)
            })
            .fold(
                HashMap::default(),
                |mut acc: HashMap<String, Service>, (id, svc)| {
                    acc.insert(id.to_string(), svc);
                    acc
                },
            );

        Ok(Compose::new(gate_services))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_serialize_to_valid_compose_file() {
        let compose = Compose::new(HashMap::default());

        let out = serde_yaml::to_string(&compose).map_err(|_| ());
        assert_eq!(
            Ok("---
version: \"3\"
services: {}
"
            .to_string()),
            out
        )
    }

    #[test]
    fn should_serialize_to_valid_service() {
        let command = Command::new("not".to_string(), vec![]);
        let service = Service::new(command, HealthCheck::default());

        let out = serde_yaml::to_string(&service).map_err(|_| ());
        assert_eq!(
            Ok("---
image: \"ghcr.io/ncatelli/gates:main\"
command: \"not -listen-addr '0.0.0.0:8080'\"
healthcheck:
  test:
    - CMD
    - curl
    - \"-f\"
    - \"http://127.0.0.1:8080/healthcheck\"
"
            .to_string()),
            out
        );

        // with single output addr

        let links = vec![("and_gate".to_string(), 'a')];
        let command = Command::new("not".to_string(), links);
        let service = Service::new(command, HealthCheck::default());

        let out = serde_yaml::to_string(&service).map_err(|_| ());
        assert_eq!(
            Ok("---
image: \"ghcr.io/ncatelli/gates:main\"
command: \"not -listen-addr '0.0.0.0:8080' -output-addrs 'http://and_gate:8080/input/a'\"
healthcheck:
  test:
    - CMD
    - curl
    - \"-f\"
    - \"http://127.0.0.1:8080/healthcheck\"
"
            .to_string()),
            out
        );

        // with multiple output addrs

        let links = vec![("and_gate".to_string(), 'a'), ("or_gate".to_string(), 'b')];
        let command = Command::new("not".to_string(), links);
        let service = Service::new(command, HealthCheck::default());

        let out = serde_yaml::to_string(&service).map_err(|_| ());
        assert_eq!(
            Ok("---
image: \"ghcr.io/ncatelli/gates:main\"
command: \"not -listen-addr '0.0.0.0:8080' -output-addrs 'http://and_gate:8080/input/a,http://or_gate:8080/input/b'\"
healthcheck:
  test:
    - CMD
    - curl
    - \"-f\"
    - \"http://127.0.0.1:8080/healthcheck\"
"
            .to_string()),
            out
        )
    }
}
