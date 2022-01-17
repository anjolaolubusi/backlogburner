import axios from 'axios';

export const MODEL_API = axios.create({
    baseURL: 'http://localhost:5000/'
})