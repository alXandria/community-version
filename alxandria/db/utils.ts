import prisma from '../db'

export async function createDocument(title: string, markdown: string) {

    const document = await prisma.document.create({
        data: {
            title: title,
            version: await newDocumentVersionNumber(title),
            markdown: markdown
        }
    })

    return document;
  }

  export async function newDocumentVersionNumber(title: string){
    
    const document = await getLatestFromTitle(title);

    if (document != null) {
      return document.version + 1
    }

    return 0
  }

  export async function getDocument(id: string) {
    
    const document = await prisma.document.findUnique({
      where: {
        id: id
      }
    });

    return document;
  }

  export async function getAllDocuments() {
    
    const documents = await prisma.document.findMany({
      orderBy: [
        {
          title: 'asc'
        }
      ]
    });

    return documents
  }

  export async function getAllDocumentIds(){
    return (await getAllDocuments()).map((document) => document.id)
  }

  export async function getAllDocumentsFromTitle(title: string) {
    const documents = await prisma.document.findMany({
      where: {
        title: title
      }
    });

    return documents
  }

  export async function getLatestFromTitle(title: string) {
    const documents = await prisma.document.findMany({
      where: {
        title: title
      },
      orderBy: [
        {
          version: 'desc'
        }
      ]
    });

    if (!documents.length) {
      return null
    }

    return documents[0];

  }