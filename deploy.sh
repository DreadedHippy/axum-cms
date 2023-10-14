echo "Axum CMS backend server..."

set -o allexport
source .env set

docker build -t $DOCKER_IMAGE_NAME .

if [ $? -eq 0 ]; then
  echo "Image built successfully"
	echo "Pushing to cloud..."
	docker push $DOCKER_IMAGE_NAME

	if [ $? -eq 0 ]; then
		echo "Image pushed successfully"
	else
		echo "Image push failed"
	fi

else
  echo "Command Failed"
fi