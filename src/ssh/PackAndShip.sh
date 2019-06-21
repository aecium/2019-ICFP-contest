part1=`dirname "$1"`
part2=`basename "$1"`
cp $1/*.sol .
zip solutions.zip *.sol
resp=$(curl -F "private_id=ba8deababaa4fa2dfabe143b" -F "file=@solutions.zip" https://monadic-lab.org/submit)
echo "resp:" $resp
mv *.sol shipped
mv solutions.zip shipped
