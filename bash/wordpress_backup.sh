#!env bash

DATE=$(date +"%Y-%m-%d")
MYSQL_DUMP="mysqldump --routines -u root -pABitBE6 --all-databases"
BACKUP_DIR="$HOME/wordpress_backups"
WORDPRESS_DIR="/www/blog.xaerolimit.net"
FILENAME="wordpress_backup"

cd $WORDPRESS_DIR
$(MYSQL_DUMP) > sqldump-$(DATE).sql

if [[ -d "$(BACKUP_DIR)" ]]; then
  cd "$HOME/$WORKING_DIR"
else
  echo "Creating " $BACKUP_DIR
  mkdir "$WORKING_DIR"
  echo "Created " $BACKUP_DIR
  cd "$WORKING_DIR"
  echo "Changed into " pwd
fi

tar -Jcf $(BACKUP_DIR)/$FILENAME-$($DATE).xz $(WORDPRESS_DIR)
echo "Backup file created in " $BACKUP_DIR "with file name of "
$FILENAME-$(DATE)
