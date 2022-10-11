S3_BUCKET_NAME="alpaca-env-files"
TMP_DIR=tmp

# if TMP_DIR does not exist, create it
if [ ! -d "$TMP_DIR" ]; then
  mkdir $TMP_DIR
fi

# cp .env from s3
aws s3 cp s3://$S3_BUCKET_NAME/.env $TMP_DIR/.env.bak
# cp .env.bak as .env
cp $TMP_DIR/.env.bak $TMP_DIR/.env

# open .env by vim
vim $TMP_DIR/.env

# check diff
diff=`diff -u $TMP_DIR/.env.bak $TMP_DIR/.env`

# echo diff, and check dialog
if [ "$diff" != "" ]; then
  echo "diff:"
  echo -n $diff
  echo "Do you want to upload .env to s3? [y/N]"
  read answer
  if [ "$answer" = "y" ]; then
    aws s3 cp $TMP_DIR/.env s3://$S3_BUCKET_NAME/.env
  else
    echo "Canceled."
  fi
else
  echo "No diff."
fi
