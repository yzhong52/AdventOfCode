for entry in day*.ts
do
  # https://stackoverflow.com/a/12152669/1035008
  DAY=$(echo $entry | cut -f 1 -d '.')
  npm run $DAY
done
