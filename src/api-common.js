/**
 * This file represents the object that sends data to the api backend
 */
//Imports axios package that sends HTTP Requests
import axios from 'axios';

//Exports axios object
export const MODEL_API = axios.create({
    baseURL: 'http://localhost:5000/'
})