nohup RUST_LOG=actix_web=info ./mashton_party_api > access.log &
ID=`ps ax -o pid,args | egrep "[m]ashton_party_api" | head -1 | sed -e 's/^[ \t]*//' | cut -d ' ' -f 1`
echo Started with PID $ID