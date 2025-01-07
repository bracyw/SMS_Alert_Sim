/**
 * Type to represent the body of a send creation post request.
 */
export type SendCreateBody = {
    msg: string | null,
    num_msgs: number,
};