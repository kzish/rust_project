-- Your SQL goes here

CREATE TABLE `tasks` (
	`id` INT(11) NOT NULL AUTO_INCREMENT,
	`description` TEXT NULL DEFAULT NULL COLLATE 'latin1_swedish_ci',
	`done` BIT(1) NULL DEFAULT NULL,
	PRIMARY KEY (`id`) USING BTREE
)
