-- Add migration script here
-- 创建用户表
CREATE TABLE `users` (
  `id` VARCHAR(36) NOT NULL,
  `username` varchar(50) NOT NULL,
  `password` varchar(100) DEFAULT NULL,
	`salt` varchar(100) DEFAULT NULL,
  `email` varchar(50) DEFAULT NULL,
  `phone` varchar(20) DEFAULT NULL,
  `status` int(1) DEFAULT NULL,
  `avatar` varchar(200) DEFAULT NULL,
  `deleted` INT(1) DEFAULT 0,
	createdAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8;


-- 注入测试账户
insert into `users` (`id`, `username`, `password`, `salt`, `email`, `phone`, `status`, `avatar`, `deleted`) values('111','admin','123456', '', 'super@aliyun.com','18677778888','1','https://wpimg.wallstcn.com/f778738c-e4f8-4870-b634-56703b4acafe.gif','0');
insert into `users` (`id`, `username`, `password`, `salt`, `email`, `phone`, `status`, `avatar`, `deleted`) values('222','zhangsan','123456', '', 'zhangsan@gmail.com','13966667777','1','https://wpimg.wallstcn.com/f778738c-e4f8-4870-b634-56703b4acafe.gif','0');
insert into `users` (`id`, `username`, `password`, `salt`, `email`, `phone`, `status`, `avatar`, `deleted`) values('333','lisi','123456', '', 'lisi@gmail.com','13966667778','1','https://wpimg.wallstcn.com/f778738c-e4f8-4870-b634-56703b4acafe.gif','0');


-- --  创建角色表
-- CREATE TABLE `roles` (
--   `role_id` int(11) NOT NULL AUTO_INCREMENT,
--   `role_name` varchar(50) DEFAULT NULL,
--   `role_desc` varchar(100) DEFAULT NULL,
--   PRIMARY KEY (`role_id`)
-- ) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4;

-- -- 测试
-- insert into `roles` (`role_id`, `role_name`, `role_desc`) values('1','admin','超级管理员');
-- insert into `roles` (`role_id`, `role_name`, `role_desc`) values('2','hr','人事专员');
-- insert into `roles` (`role_id`, `role_name`, `role_desc`) values('3','normal','普通员工');


-- 用户角色映射表
-- CREATE TABLE `userRoles` (
--   `id` int(11) NOT NULL AUTO_INCREMENT,
--   `user_id` int(11) DEFAULT NULL,
--   `role_id` int(11) DEFAULT NULL,
--   PRIMARY KEY (`id`)
-- ) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4;

-- 测试
-- insert into `userRoles` (`id`, `user_id`, `role_id`) values('1','1','1');
