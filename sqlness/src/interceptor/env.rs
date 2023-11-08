// Copyright 2023 CeresDB Project Authors. Licensed under Apache-2.0.

use std::collections::HashMap;

use crate::case::QueryContext;
use crate::interceptor::{Interceptor, InterceptorFactory, InterceptorRef};

const PREFIX: &str = "ENV";

/// Read environment variables and fill them in query.
///
/// # Example
/// ``` sql
/// -- SQLNESS ENV SECRET
/// SELECT $SECRET;
/// ```
///
/// Environment variables declared in `ENV` interceptor will be replaced in the
/// going to be executed. It won't be rendered in the result file so you can
/// safely put secret things in your query.
///
/// Note that only decalred and present environment variables will be replaced.
///
/// You can either declare multiple env in one intercetor or separate them into
/// different interceptors. The following two examples are equivalent:
///
/// ``` sql
/// -- SQLNESS ENV SECRET1 SECRET2
/// SELECT $SECRET1, $SECRET2;
///
/// -- SQLNESS ENV SECRET1
/// -- SQLNESS ENV SECRET2
/// SELECT $SECRET1, $SECRET2;
/// ````
#[derive(Debug)]
pub struct EnvInterceptor {
    /// Environment variables to be replaced.
    data: HashMap<String, String>,
}

impl Interceptor for EnvInterceptor {
    fn before_execute(&self, execute_query: &mut Vec<String>, _: &mut QueryContext) {
        for line in execute_query {
            for (key, value) in &self.data {
                let rendered = line.replace(key, value);
                *line = rendered;
            }
        }
    }
}

pub struct EnvInterceptorFactory;

impl InterceptorFactory for EnvInterceptorFactory {
    fn try_new(&self, interceptor: &str) -> Option<InterceptorRef> {
        Self::create(interceptor).map(|i| Box::new(i) as InterceptorRef)
    }
}

impl EnvInterceptorFactory {
    fn create(interceptor: &str) -> Option<EnvInterceptor> {
        if interceptor.starts_with(PREFIX) {
            let input = interceptor
                .trim_start_matches(PREFIX)
                .trim_start()
                .trim_end();
            let envs = input.split(' ').collect::<Vec<_>>();

            let mut env_data = HashMap::new();
            for env in envs {
                if let Ok(value) = std::env::var(env) {
                    env_data.insert(format!("${env}"), value);
                }
            }

            Some(EnvInterceptor { data: env_data })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cut_env_string() {
        let input = "ENV SECRET NONEXISTENT";
        std::env::set_var("SECRET", "2333");

        let expected = [("$SECRET".to_string(), "2333".to_string())]
            .into_iter()
            .collect();

        let interceptor = EnvInterceptorFactory::create(input).unwrap();
        assert_eq!(interceptor.data, expected);
    }
}
