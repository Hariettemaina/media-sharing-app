document.addEventListener('DOMContentLoaded', function () {
    const uploadForm = document.getElementById('upload-form');
    const fileInput = document.getElementById('file-input');
    const mediaGrid = document.querySelector('.media-grid');

    let websocket;
    const uploadedImages = new Set();

    
    const currentUserID = localStorage.getItem("userId");

    function initWebSocket() {
        websocket = new WebSocket('ws://localhost:8080', 'graphql-ws');

        websocket.onopen = function () {
            console.log('WebSocket connection established');
            websocket.send(JSON.stringify({ type: 'connection_init', payload: {} }));
            startSubscription();
        };
        websocket.onmessage = function (event) {
            console.log('WebSocket message received:', event.data);
            const message = JSON.parse(event.data);
            handleWebSocketMessage(message);
        };
        websocket.onclose = function () {
            console.log('WebSocket connection closed');
        };
        websocket.onerror = function (error) {
            console.error('WebSocket error:', error);
        };
    }

    function handleWebSocketMessage(message) {
        console.log('Handling WebSocket message:', message);
        if (message.type === 'data' && message.id === '1') {
            const mediaUpdate = message.payload.data.mediaUpdates;
            const imageUrl = extractImageUrl(mediaUpdate.message);
            const uploaderUserId = mediaUpdate.userId;
            // const uploaderUserName = mediaUpdate.userName;
            alert(`${uploaderUserId} (ID: ${uploaderUserId}) has uploaded an image!`);


            appendImageToGrid(imageUrl, false);

            
        }
    }

    function extractImageUrl(message) {
        const matches = message.match(/Path: (.+?)\n/);
        if (matches && matches[1]) {
            return `http://localhost:8080/${matches[1].trim()}`;
        }
        return '';
    }

    function startSubscription() {
        websocket.send(JSON.stringify({
            id: '1',
            type: 'start',
            payload: {
                query: `
                    subscription {
                        mediaUpdates {
                            userId
                            message
                        }
                    }
                `,
                variables: {}
            }
        }));
    }

    // function alertUser(userId, imageUrl) {
    //     alert(`User ${userId} has uploaded a new image: ${imageUrl}`);
    // }

    uploadForm.addEventListener('submit', async function (event) {
        event.preventDefault();
        const userId = localStorage.getItem("userId");
        if (!userId) {
            alert('Please login first');
            return;
        }
        if (!fileInput.files.length) {
            alert('Please select a file to upload.');
            return;
        }
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
                    userId: parseInt(userId)
                }
            }
        }));
        formData.append('map', JSON.stringify({ '0': ['variables.input.image'] }));
        formData.append('0', fileInput.files[0]);
        try {
            const response = await fetch('http://localhost:8080', { method: 'POST', body: formData });
            const data = await response.json();
            if (data.errors) {
                console.error('Upload failed:', data.errors);
                alert('Upload failed. Please try again.');
            } else if (data.data && data.data.images.upload) {
                const imageData = data.data.images.upload;
                // appendImageToGrid(imageData, false); 
            }
        } catch (error) {
            console.error('Error:', error);
            alert('Error uploading the file. Please try again.');
        }
    });

    function loadStoredImages() {
        const storedImages = JSON.parse(localStorage.getItem('uploadedimages')) || [];
        storedImages.forEach(imageUrl => appendImageToGrid(imageUrl, true));
    }

    function appendImageToGrid(imageUrl, isInitialLoad = false) {
        if (imageUrl && !uploadedImages.has(imageUrl)) {
            console.log('Appending image: ${imageUrl}');
            const imgElement = document.createElement('img');
            imgElement.src = imageUrl;
            imgElement.style.width = '100%';
            imgElement.style.height = 'auto';
            mediaGrid.appendChild(imgElement);
            uploadedImages.add(imageUrl);
            if (!isInitialLoad) {
                storeImageInLocal(imageUrl);
            }
        } else {
            console.log(`Image URL already exists: ${imageUrl}`); // Debugging log
        }
    }


    function storeImageInLocal(imageUrl) {
        const storedImages = JSON.parse(localStorage.getItem('uploadedimages')) || [];
        if (!storedImages.includes(imageUrl)) {
            storedImages.push(imageUrl);
            localStorage.setItem('uploadedimages', JSON.stringify(storedImages));
        }
    }

    initWebSocket();
    loadStoredImages();
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







