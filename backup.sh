# sudo crontab -e
# Every day at 04:00
# 0 4 * * * /home/avskjerming/Documents/Avskjerming.Calendar/backup.sh

USB_DIR=/media/avskjerming/ElementsSE
BACKUPS_DIR=$USB_DIR/backups

# Make sure usb exists
MOUNTPOINT=$(mountpoint "$USB_DIR")
if [ "$MOUNTPOINT" != "$USB_DIR is a mountpoint" ]; then
    echo "ERROR $(date +'%Y-%m-%dT%H:%M:%S'): Usb not found" >> backup.log
    exit 0
fi

# Create backups directory if not exists
[ ! -d "$BACKUPS_DIR" ] && mkdir "$BACKUPS_DIR"

# Copy db to hard drive
NAME=$BACKUPS_DIR/db_$(date +'%Y-%m-%dT%H-%M-%S').zip
sudo zip -r $NAME /home/avskjerming/Documents/Avskjerming.Calendar/db

# Only keep 10 latest backups
# if [[ $(ls $BACKUPS_DIR | wc -l) -gt 10 ]]; then
#     ls -t1 $BACKUPS_DIR | tail -n +11 | xargs -I{} sudo rm -rf $BACKUPS_DIR/{}
# fi
