CREATE TABLE Users (
	id int NOT NULL AUTO_INCREMENT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	name varchar(255),
    email varchar(255),
    password varchar(255),
    primary key (id)
);

