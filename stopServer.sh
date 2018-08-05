ID=`ps ax -o pid,args | egrep "[m]ashton_party_api" | head -1 | sed -e 's/^[ \t]*//' | cut -d ' ' -f 1`
echo 'Killing pid '$ID
kill -kill $ID