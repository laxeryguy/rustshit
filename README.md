# rustshit
i learn rust because i'm femboy/gay/furideer/camera/pc/laptop/phone/cat/dog

DELIMITER $$

CREATE PROCEDURE create_users_from_table()
BEGIN
    DECLARE done INT DEFAULT 0;
    DECLARE v_imie VARCHAR(255);
    DECLARE v_nazwisko VARCHAR(255);
    DECLARE v_password VARCHAR(255);
    DECLARE v_select_b TINYINT;
    DECLARE v_insert_b TINYINT;
    DECLARE v_update_b TINYINT;
    DECLARE v_delete_b TINYINT;
    DECLARE v_create_b TINYINT;
    DECLARE v_drop_b TINYINT;
    DECLARE v_alter_b TINYINT;
    DECLARE v_index_b TINYINT;
    DECLARE v_grant_b TINYINT;
    DECLARE v_super_b TINYINT;

    DECLARE cur CURSOR FOR
        SELECT u.imie, u.nazwisko, u.password_p,
               g.select_b, g.insert_b, g.update_b, g.delete_b,
               g.create_b, g.drop_b, g.alter_b, g.index_b,
               g.grant_b, g.super_b
        FROM users u
        JOIN grupa g ON u.typ_konta = g.id;

    DECLARE CONTINUE HANDLER FOR NOT FOUND SET done = 1;

    OPEN cur;

    read_loop: LOOP
        FETCH cur INTO v_imie, v_nazwisko, v_password,
                       v_select_b, v_insert_b, v_update_b, v_delete_b,
                       v_create_b, v_drop_b, v_alter_b, v_index_b,
                       v_grant_b, v_super_b;

        IF done THEN
            LEAVE read_loop;
        END IF;

        -- Создаём пользователя (имя = imie_nazwisko)
        SET @username = CONCAT(v_imie, '_', v_nazwisko);
        SET @sql = CONCAT('CREATE USER IF NOT EXISTS ''', @username, '''@''localhost'' IDENTIFIED BY ''', v_password, '''');
        PREPARE stmt FROM @sql;
        EXECUTE stmt;
        DEALLOCATE PREPARE stmt;

        -- Выдаём права на основе флагов группы
        SET @grants = '';
        IF v_select_b  = 1 THEN SET @grants = CONCAT(@grants, 'SELECT, ');     END IF;
        IF v_insert_b  = 1 THEN SET @grants = CONCAT(@grants, 'INSERT, ');     END IF;
        IF v_update_b  = 1 THEN SET @grants = CONCAT(@grants, 'UPDATE, ');     END IF;
        IF v_delete_b  = 1 THEN SET @grants = CONCAT(@grants, 'DELETE, ');     END IF;
        IF v_create_b  = 1 THEN SET @grants = CONCAT(@grants, 'CREATE, ');     END IF;
        IF v_drop_b    = 1 THEN SET @grants = CONCAT(@grants, 'DROP, ');       END IF;
        IF v_alter_b   = 1 THEN SET @grants = CONCAT(@grants, 'ALTER, ');      END IF;
        IF v_index_b   = 1 THEN SET @grants = CONCAT(@grants, 'INDEX, ');      END IF;
        IF v_grant_b   = 1 THEN SET @grants = CONCAT(@grants, 'GRANT OPTION, '); END IF;
        IF v_super_b   = 1 THEN SET @grants = CONCAT(@grants, 'SUPER, ');      END IF;

        -- Убираем последнюю запятую
        IF @grants != '' THEN
            SET @grants = LEFT(@grants, LENGTH(@grants) - 2);
            SET @sql = CONCAT('GRANT ', @grants, ' ON *.* TO ''', @username, '''@''localhost''');
            PREPARE stmt FROM @sql;
            EXECUTE stmt;
            DEALLOCATE PREPARE stmt;
        END IF;

    END LOOP;

    CLOSE cur;

    FLUSH PRIVILEGES;
END$$

DELIMITER ;

-- Запускаем
CALL create_users_from_table();