document.addEventListener('DOMContentLoaded', function () {
    const uploadForm = document.getElementById('upload-form');
    const fileInput = document.getElementById('file-input');
    const mediaGrid = document.querySelector('.media-grid'); 

    uploadForm.addEventListener('submit', function (event) {
        event.preventDefault();

        const formData = new FormData();
        formData.append('operations', JSON.stringify({
            query: `
                mutation UploadFile($input: UploadUserInput!) {
                    images {
                        upload(input: $input)
                    }
                }
            `,
            variables: {
                input: {
                    image: null,
                    userId: 1
                }
            }
        }));
        formData.append('map', JSON.stringify({
            '0': ['variables.input.image']
        }));
        formData.append('0', fileInput.files[0]);

        fetch('http://localhost:8000', {
            method: 'POST',
            body: formData,
        })
            .then(response => response.json())
            .then(data => {
                console.log('Full response;', data);
                if (data.data && data.data.images.upload) {
                    const imageData = data.data.images.upload;

                    const imgElement = document.createElement('img');
                    imgElement.src = imageData;
                    imgElement.style.width = '200px';

                    mediaGrid.appendChild(imgElement);
                    const storedimages = JSON.parse(localStorage.getItem('uploadedimages')) || [];
                    storedimages.push(imageData);
                    localStorage.setItem('uploadedimages', JSON.stringify(storedimages));
                    console.log('Stored images:', storedimages); 

                } else {
                    console.error('Upload failed', data);
                }
            })
            .catch(error => {
                console.error('Error:', error);
            });
    });

    function loadStoredImages() {
        const storedImages = JSON.parse(localStorage.getItem('uploadedimages')) || [];
        console.log('Retrieved stored images:', storedImages); 

        storedImages.forEach(url => {
            const imageElement = document.createElement('img');
            imageElement.src = url;
            imageElement.style.width = '200px';
            mediaGrid.appendChild(imageElement);
        });
    }

    loadStoredImages(); 
    function loadImagesFromServer() {
        fetchImagesFromServer().then(images => {
            images.forEach(imageData => {
                const imgElement = document.createElement('img');
                imgElement.src = imageData;
                imgElement.style.width = '200px';
                mediaGrid.appendChild(imgElement);
            });
        }).catch(error => {
            console.error('Error fetching images:', error);
        });
    }

    async function fetchImagesFromServer() {
        try {
            const response = await fetch('http://localhost:8000', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Accept': 'application/json',
                },
                body: JSON.stringify({
                    query: `
                        query GetImages {
                            images {
                                getImages 
                            }
                        }
                    `
                }),
            });

            const data = await response.json();

            if (data.data && Array.isArray(data.data.images)) {
                return data.data.images;
            } else {
                throw new Error('No images found');
            }
        } catch (error) {
            console.error('Error fetching images:', error);
            throw error;
        }
    }

    loadImagesFromServer();
});





// function getUserIdFromCookie() {
//     var key, value, i;
//     var cookieArray = document.cookie.split(';');

//     for (i = 0; i < cookieArray.length; i++) {
//         key = cookieArray[i].slice(0, cookieArray[i].indexOf("="));
//         value = cookieArray[i].slice(cookieArray[i].indexOf("=") + 1);

//         if (key == 'id') {
//             return parseInt(value, 10);
//         }
//     }
//     return null;
// }

// const id = getUserIdFromCookie();
// if (id === null) {
//     console.error('User ID not found in cookies.');
//     return;
// }




// 4. *Backpressure Handling with RabbitMQ*
//    - *Description:* Integrate RabbitMQ to handle backpressure when processing media files.
//    - *Expected Functionality:* RabbitMQ queues are used to manage the flow of media processing tasks. Workers consume tasks from the queue and process them asynchronously.
//    - *Criteria for Completion:* RabbitMQ is integrated into the system to handle backpressure effectively.
//    - *Test Suites:*
//      - Test RabbitMQ integration.
//      - Test asynchronous processing of media tasks.

// 5. *Real-time Updates with WebSockets*
//    - *Description:* Implement real-time updates for users using WebSockets.
//    - *Expected Functionality:* Users receive real-time notifications when new media is uploaded or when there are updates to their uploaded media.
//    - *Criteria for Completion:* Real-time updates are implemented using WebSockets, and users receive notifications in real-time.
//    - *Test Suites:*
//      - Test WebSocket connection establishment.
//      - Test real-time notifications for media uploads and updates.








// document.addEventListener('DOMContentLoaded', function () {
//     const uploadForm = document.getElementById('upload-form');
//     const fileInput = document.getElementById('file-input');
//     const mediaGrid = document.querySelector('.media-grid');

//     uploadForm.addEventListener('submit', function (event) {
//         event.preventDefault();

//         const formData = new FormData();
//         formData.append('operations', JSON.stringify({
//             query: `
//                 mutation UploadFile($input: UploadUserInput!) {
//                     images {
//                         upload(input: $input)
//                     }
//                 }
//             `,
//             variables: {
//                 input: {
//                     image: null,
//                     userId: 1
//                 }
//             }
//         }));
//         formData.append('map', JSON.stringify({
//             '0': ['variables.input.image']
//         }));
//         formData.append('0', fileInput.files[0]);

//         fetch('http://localhost:8000', {
//             method: 'POST',
//             body: formData,
//         })
//            .then(response => response.json())
//            .then(data => {
//                 console.log('Full response;', data);
//                 if (data.data && data.data.images.upload) {
//                     const imageData = data.data.images.upload;

//                     const imgElement = document.createElement('img');
//                     imgElement.src = imageData;
//                     imgElement.style.width = '200px';

//                     mediaGrid.appendChild(imgElement);
//                 } else {
//                     console.error('Upload failed', data);
//                 }
//             })
//            .catch(error => {
//                 console.error('Error:', error);
//             });
//     });

//     function loadImagesFromServer(userId) {
//         fetchImagesFromServer(userId).then(images => {
//             images.forEach(imageData => {
//                 const imgElement = document.createElement('img');
//                 imgElement.src = imageData;
//                 imgElement.style.width = '200px';
//                 mediaGrid.appendChild(imgElement);
//             });
//         }).catch(error => {
//             console.error('Error fetching images:', error);
//         });
//     }

//     async function fetchImagesFromServer(userId) {
//         try {
//             const response = await fetch('http://localhost:8000', {
//                 method: 'POST',
//                 headers: {
//                     'Content-Type': 'application/json',
//                     'Accept': 'application/json',
//                 },
//                 body: JSON.stringify({
//                     query: `
//                         query GetImages($userId: Int!) {
//                             images(userId: $userId) {
//                                 getImages 
//                             }
//                         }
//                     `,
//                     variables: {
//                         userId: userId
//                     }
//                 }),
//             });

//             const data = await response.json();

//             if (data.data && Array.isArray(data.data.images)) {
//                 return data.data.images;
//             } else {
//                 throw new Error('No images found');
//             }
//         } catch (error) {
//             console.error('Error fetching images:', error);
//             throw error;
//         }
//     }

//     // Example usage: Load images for a specific user
//     loadImagesFromServer(1); // Replace 1 with the actual user ID
// });
