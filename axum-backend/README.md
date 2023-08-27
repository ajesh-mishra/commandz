## DataBase set-up

Reference Article: (Docker Postgres)[https://geshan.com.np/blog/2021/12/docker-postgres/]

- Download and Run a Postgress Docker Image

```bash
docker run --name commandz-postgres \
 --rm \
 -e POSTGRES_USER=postgres \
 -e POSTGRES_PASSWORD=postgres \
 -e PGDATA=/var/lib/postgresql/data/pgdata \
 -v /Users/ajesh/code/commandz/postgres-database:/var/lib/postgresql/data \
 -p 5432:5432 \
 -it postgres:15.2-alpine
```

- Go inside the container

```bash
docker exec -it commandz-postgres /bin/sh
psql --username postgres
```

CREATE DATABASE commandz;

\c commandz

CREATE TABLE commands (
	id serial PRIMARY KEY,
	command_name VARCHAR ( 250 ) UNIQUE NOT NULL,
	description VARCHAR ( 250 ) NOT NULL,
    command_type VARCHAR ( 50 ) NOT NULL,
	created_on TIMESTAMP NOT NULL
);


\dt


INSERT INTO 
    commands (command_name, description, command_type, created_on)
VALUES
    ('pwd','present working directory','bash',CURRENT_TIMESTAMP),
    ('id','shows the id','bash',CURRENT_TIMESTAMP);

 
SELECT * FROM commands;


https://levelup.gitconnected.com/creating-and-filling-a-postgres-db-with-docker-compose-e1607f6f882f

https://www.postgresqltutorial.com/postgresql-administration/postgresql-show-tables/

