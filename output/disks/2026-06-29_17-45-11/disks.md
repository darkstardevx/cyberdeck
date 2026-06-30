<div style='background:#6a0dad;color:white;padding:6px;'>💾 DISK INTELLIGENCE SYSTEM</div>

Timestamp: Mon Jun 29 05:45:11 PM PDT 2026
## 📦 BLOCK DEVICES

## ⚡ NVME SUMMARY
Node                  Generic               SN                   Model                                    Namespace  Usage                      Format           FW Rev  
--------------------- --------------------- -------------------- ---------------------------------------- ---------- -------------------------- ---------------- --------
/dev/nvme0n1          /dev/ng0n1            8E59073B189C00020376 SPCC M.2 PCIe SSD                        0x1          1.02  TB /   1.02  TB    512   B +  0 B   EHFMC0.0

## 📦 RAW SNAPSHOT
NAME        FSTYPE FSVER LABEL     UUID                                 FSAVAIL FSUSE% MOUNTPOINTS
sda                                                                                    
└─sda1                                                                                 
sdb                                                                                    
└─sdb1      btrfs        VirtualX  c3fd65c6-247c-47db-bef2-1f5d0ea78acb     26G    88% /media/VirtualX.btrfs
sdc                                                                                    
└─sdc1      btrfs        XDrive    a739f6b6-b771-4fc8-a144-efa3417501ac   15.8G    98% /media/XDrive.btrfs
sdd                                                                                    
└─sdd1      btrfs        VolumeX   391d5516-1488-4fa7-bd30-ceca3c43e711   74.9G    68% /media/VolumeX.btrfs
sde                                                                                    
└─sde1      btrfs        Vault     c3a5ce8a-f0ff-4c10-8c8e-5d2fdeabe0ad   49.3G    97% /media/Vault.btrfs
sdf                                                                                    
└─sdf1      btrfs        Darkdrive adbcda76-9d2f-4f08-b592-de41d15f4180   34.8G    93% /media/Darkdrive.btrfs
nvme0n1                                                                                
├─nvme0n1p1 vfat   FAT32           CD8D-1D57                               7.5G     6% /media/nvme0n1p1.vfat
│                                                                                      /boot
├─nvme0n1p2 swap   1               8b1b1443-d353-48e5-a30d-d2b3260aa91c                [SWAP]
└─nvme0n1p3 btrfs                  29ff9366-828b-4529-822a-de3efc639e5b  614.8G    30% /media/nvme0n1p3.btrfs
                                                                                       /var
                                                                                       /home
                                                                                       /.snapshots
                                                                                       /
Filesystem      Size  Used Avail Use% Mounted on
/dev/nvme0n1p3  882G  266G  615G  31% /
devtmpfs         32G     0   32G   0% /dev
tmpfs            32G  169M   32G   1% /dev/shm
efivarfs        128K   38K   86K  31% /sys/firmware/efi/efivars
tmpfs            13G  4.9M   13G   1% /run
tmpfs           4.0G   80M  4.0G   2% /tmp
/dev/nvme0n1p3  882G  266G  615G  31% /.snapshots
/dev/nvme0n1p3  882G  266G  615G  31% /home
/dev/nvme0n1p3  882G  266G  615G  31% /var
tmpfs            32G  1.2G   31G   4% /var/log
/dev/nvme0n1p1  8.0G  510M  7.5G   7% /boot
/dev/nvme0n1p3  882G  266G  615G  31% /media/nvme0n1p3.btrfs
/dev/sdb1       224G  197G   26G  89% /media/VirtualX.btrfs
/dev/sdc1       932G  915G   16G  99% /media/XDrive.btrfs
/dev/sdd1       239G  163G   75G  69% /media/VolumeX.btrfs
/dev/sde1       1.9T  1.8T   50G  98% /media/Vault.btrfs
/dev/sdf1       477G  442G   35G  93% /media/Darkdrive.btrfs
tmpfs           6.3G   60M  6.3G   1% /run/user/1000
none            1.0M     0  1.0M   0% /run/credentials/systemd-journald.service
none            1.0M     0  1.0M   0% /run/credentials/systemd-resolved.service

✔ disk scan complete
