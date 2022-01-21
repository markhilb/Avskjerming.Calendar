# sudo crontab -e
# Every day at 04:00
# 0 4 * * * /home/markus/Documents/Avskjerming/Avskjerming.Calendar/backup.sh

USB_DIR="/home/markus/Documents/Avskjerming/Avskjerming.Calendar/usb"
BACKUPS_DIR="$USB_DIR/backups"

HARD_DRIVE_NAME="DataTraveler"
MOUNT_PATH="$(sudo fdisk -l | grep -B1 $HARD_DRIVE_NAME | head -n 1 | cut -d ' ' -f 2 | sed 's/://')"

if [ -z $MOUNT_PATH ]; then
    echo "ERROR $(date +'%Y-%m-%dT%H:%M:%S'): Mount path not found" >> backup.log
    exit 0
fi

# Mount hard drive
sudo mount $MOUNT_PATH $USB_DIR

# Create backups directory if not exists
[ ! -d $BACKUPS_DIR ] && sudo mkdir $BACKUPS_DIR

# Copy db to hard drive
name="db_$(date +'%Y-%m-%dT%H:%M:%S')"
sudo cp -r /home/markus/Documents/Avskjerming/Avskjerming.Calendar/db $BACKUPS_DIR/$name

# Only keep 10 latest backups
if [[ $(ls $BACKUPS_DIR | wc -l) -gt 10 ]]; then
    ls -t1 $BACKUPS_DIR | tail -n +11 | xargs -I{} sudo rm -rf $BACKUPS_DIR/{}
fi

# Unmount hard drive
sudo umount $MOUNT_PATH
