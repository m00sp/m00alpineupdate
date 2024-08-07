#!/bin/sh
# ----------------------------------------
# Bootstrap the start of a penrose session
# >> This get's run on restart as well!
# ----------------------------------------

#Certificando que rode só uma vez
pid=$$
pgrep -f m00update.sh | grep -v "^$pid$" |xargs -I{} kill {}

notify-send -u critical -t 10000 "Estamos fazendo apk update"
sudo apk update
notify-send -u critical -t 10000 "Perfeito, agora vamos a checar por novas atualizações"
#sudo apk list -u > list.txt
apks=$(sudo apk upgrade -vvs | wc -l)
dif=1
result=$(echo "$apks - $dif" | bc)
notify-send -u critical -t 10000 "Você tem $result pacatos desatualizados"
exit 0
