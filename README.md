# YoLM
Yo Login Manager(as in *Yo* from saluting another person) for sway

```
> yo YoLM, let log me into sway, bro
< okie dokie
> thxxx
```

# Instructions
Link the binary to `/usr/bin/yolm`

```bash
cargo build
sudo ln -sf /path/to/yolm/target/debug/yolm /usr/bin/yolm
```

Copy the unit service file and enable it:

```bash
sudo cp path/to/yolm/data/yolm.service /etc/systemd/system/yolm.service
systemctl enable yolm
```

Copy the PAM file:

```bash
sudo cp path/to/yolm/data/yolm /etc/pam.d/yolm
```
