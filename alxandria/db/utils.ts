import prisma from '../db'

export async function createDocument(title: string, version: string, markdown: string) {

  const document = await prisma.document.create({
      data: {
          title: title,
          version: version,
          markdown: markdown,
          date: new Date().toString()
      }
  })

  return document;
  }

export async function updateDocument(id: string, markdown: string){
  const updateDocument = await prisma.document.update({
    where: {
      id: id
    },
    data: {
      markdown: markdown
    }
  });
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