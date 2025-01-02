const backendURL = 'http://localhost:4201'

/* System */
const postNewSenderPool = () => `${backendURL}/system/sender-pool/new`;

/* Alert */
const postSendCreate = () => `${backendURL}/alert/send`;

/* Progress */
const getProgressData = () => `${backendURL}/progress-monitor/all-info`;

/* Historical */
const getAllHistoryData = () => `${backendURL}/historical/all`;

export const urls = {
    postNewSenderPool,
    postSendCreate,
    getProgressData,
    getAllHistoryData
};
