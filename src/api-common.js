import axios from 'axios';

export const ILP_API = axios.create({
    baseURL: 'http://localhost:5000/'
})