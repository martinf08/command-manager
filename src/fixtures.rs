pub fn generate_data() -> (Vec<String>, Vec<Vec<(String, String)>>) {
    let folders = vec![
        "ssh".to_string(),
        "navigation".to_string(),
        "docker".to_string(),
    ];

    let commands = vec![
        vec![
            ("ssh app@domain.tld".to_string(), "ssh:app".to_string()),
            ("ssh db@domain.tld".to_string(), "ssh:db".to_string()),
            ("ssh preprod@domain.tld".to_string(), "ssh:preprod".to_string()),
        ],
        vec![
            ("cd ~/ && $SHELL".to_string(), "nav:home".to_string()),
            ("cd /tmp && $SHELL".to_string(), "nav:home".to_string()),
            ("cd /var/www/super/app".to_string(), "nav:app".to_string()),
        ],
        vec![
            ("docker run --name alpine -it alpine".to_string(), "docker:alpine".to_string()),
            ("docker run --name ubuntu -it ubuntu".to_string(), "docker:ubuntu".to_string()),
            (r#"echo "Removing containers :" && if [ -n "$(docker container ls -aq)" ]; then docker container stop $(docker container ls -aq); docker container rm $(docker container ls -aq); fi; echo "Removing images :" && if [ -n "$(docker images -aq)" ]; then docker rmi -f $(docker images -aq); fi; echo "Removing volumes :" && if [ -n "$(docker volume ls -q)" ]; then docker volume rm $(docker volume ls -q); fi; echo "Removing networks :" && if [ -n "$(docker network ls | awk '{print $1" "$2}' | grep -v 'ID\|bridge\|host\|none' | awk '{print $1}')" ]; then docker network rm $(docker network ls | awk '{print $1" "$2}' | grep -v 'ID\|bridge\|host\|none' | awk '{print $1}'); fi;"#.to_string(),
             "docker:clean".to_string()
            ),
        ],
    ];

    (folders, commands)
}
