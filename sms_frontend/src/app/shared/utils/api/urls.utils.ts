const backendURL = 'http://localhost:4201'

/* System */
const postNewSenderPoolURL = () => `${backendURL}/system/sender-pool/new`;

/* Alert */
const postSendCreateURL = () => `${backendURL}/alert/send`;

/* Progress */
const getProgressDataURL = () => `${backendURL}/progress-monitor/all-info`;

/* Historical */
const getAllHistoryDataURL = () => `${backendURL}/historical/all`;

export const urls = {
    postNewSenderPoolURL,
    postSendCreateURL,
    getProgressDataURL,
    getAllHistoryDataURL
};
