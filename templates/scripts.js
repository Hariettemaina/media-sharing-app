document.addEventListener('DOMContentLoaded', function () {
    const uploadForm = document.getElementById('upload-form');
    const fileInput = document.getElementById('file-input');

    uploadForm.addEventListener('submit', function (event) {
        event.preventDefault();

        const userId = getUserIdFromCookie();
        if (userId === null) {
            console.error('User ID not found in cookies.');
            return;
        }
        const formData = new FormData();
        formData.append('operations', JSON.stringify({
            query:  `
        mutation UploadFile($input: UploadUserInput!) {
            images {
                upload(input: $input)
            }
        }
    `,
            variables: {
                input: {
                    image: null, 
                    userId: userId
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
            if (data.data && data.data.uploadMedia && data.data.uploadMedia.success) {
                console.log('Upload successful');
            } else {
                console.error('Upload failed', data);
            }
        })
        .catch(error => {
            console.error('Error:', error);
        });
    });
});


function getUserIdFromCookie() {
    var key, value, i;
    var cookieArray = document.cookie.split(';');

    for (i = 0; i < cookieArray.length; i++) {
        key = cookieArray[i].slice(0, cookieArray[i].indexOf("="));
        value = cookieArray[i].slice(cookieArray[i].indexOf("=") + 1);

        if (key == 'userID') {
            return parseInt(value, 10);
        }
    }
    return null;
}

// document.getElementById('uploadForm').addEventListener('submit', function (event) {
//     event.preventDefault();

//     const fileInput = document.getElementById('fileInput');
//     const userId = document.getElementById('userId').value;
//     const file = fileInput.files[0];

//     if (!file) {
//         alert('Please select a file to upload.');
//         return;
//     }

//     const formData = new FormData();
//     formData.append('image', file);
//     formData.append('user_id', userId);

//     const mutation = `
//         mutation UploadFile($input: UploadUserInput!) {
//             images {
//                 upload(input: $input)
//             }
//         }
//     `;


//     const body = new FormData();
//     body.append('operations', JSON.stringify({
//         query: mutation,
//         variables: {
//             input: {
//                 image: null,
//                 user_id: userId
//             }
//         }
//     }));
//     body.append('map', JSON.stringify({
//         "0": ["variables.input.image"]
//     }));
//     body.append('0', file);
    

//     fetch('http://localhost:8000', {
//         method: 'POST',
//         body: body
//     })
//         .then(response => response.json())
//         .then(data => {
//             if (data.errors) {
//                 console.error('Error uploading file:', data.errors);
//                 alert('Error uploading file.');
//             } else {
//                 console.log('File uploaded successfully:', data);
//                 alert('File uploaded successfully.');

//                 const imageUrl = data.data.images.upload.imageUrl;
//                 const imageElement = document.createElement('img');
//                 imageElement.src = imageUrl;

//                 const container = document.getElementById('image-container');
//                 container.appendChild(imageElement);
//             }
//         })
//         .catch(error => {
//             console.error('Error:', error);
//             alert('Error uploading file.');
//         });
// });


// headers: {
//     'Content-Type': 'application/json',
//     'Accept': 'application/json',
// },

// fetch('http://localhost:8000', {
//     method: 'POST',
//     body: body
// })
// .then(response => response.json())
// .then(data => {
//     if (data.errors) {
//         console.error('Error uploading file:', data.errors);
//         alert('Error uploading file.');
//     } else {
//         console.log('File uploaded successfully:', data);
//         alert('File uploaded successfully.');

//         // Assuming the server returns the image URL in the response
//         const imageUrl = data.data.upload.imageUrl; // Adjust this line based on the actual response structure

//         // Create an <img> element and set its src to the image URL
//         const imageElement = document.createElement('img');
//         imageElement.src = imageUrl;

//         // Append the <img> element to the container
//         const container = document.getElementById('image-container');
//         container.appendChild(imageElement);
//     }
// })
// .catch(error => {
//     console.error('Error:', error);
//     alert('Error uploading file.');
// });
