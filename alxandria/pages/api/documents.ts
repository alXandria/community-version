import type { NextApiRequest, NextApiResponse } from "next";
import prisma from "../../db";

export default async (req: NextApiRequest, res: NextApiResponse) => {
    if (req.method !== 'POST') {
        return res.status(405).json({ message: 'Method not allowed'});
    }

    const documentData = JSON.parse(req.body);

    const savedDocument = await prisma.document.create({
        data: documentData
    })

    res.json(savedDocument);
}