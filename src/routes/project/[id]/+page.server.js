
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
export function load({params}) {
    console.log(params);
    return {
        id: params.id
    };
}