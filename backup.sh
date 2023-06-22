# sudo crontab -e
# Every day at 04:00
# 0 4 * * * /home/markus/Documents/Avskjerming/Avskjerming.Calendar/backup.sh

USB_DIR=/media/avskjerming/ElementsSE
BACKUPS_DIR="$USB_DIR/backups"

MOUNTPOINT=$(mountpoint "$USB_DIR")
if [ "$MOUNTPOINT" != "$USB_DIR is a mountpoint" ]; then
    echo "ERROR $(date +'%Y-%m-%dT%H:%M:%S'): Usb not found" >> backup.log
    exit 0
fi

# Create backups directory if not exists
[ ! -d $BACKUPS_DIR ] && mkdir $BACKUPS_DIR

# Copy db to hard drive
name="db_$(date +'%Y-%m-%dT%H:%M:%S')"
PGPASSWORD=$PGPASSWORD pg_dump -h localhost -U postgres -Fd -f $BACKUPS_DIR/$name 2> backup.log

# Only keep 10 latest backups
# if [[ $(ls $BACKUPS_DIR | wc -l) -gt 10 ]]; then
#     ls -t1 $BACKUPS_DIR | tail -n +11 | xargs -I{} sudo rm -rf $BACKUPS_DIR/{}
# fi
