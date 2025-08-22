import sharp from "sharp";

//@ts-ignore
export async function GET({ params, request }) {
    const buffer = await sharp("public/favicon.svg")
        .resize(params.width)
        .ensureAlpha()
        .flatten({ background: { r: 0, g: 0, b: 0, alpha: 0 } })
        .toBuffer();
    //@ts-ignore
    return new Response(buffer, {
        headers: { "Content-Type": "image/png" }
    });
}

export function getStaticPaths() {
    return [
        { params: { apple: "apple-icon", width: 180 } },
    ];
}
