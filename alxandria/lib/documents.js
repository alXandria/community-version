// Using local file system as database for now
import fs from 'fs';
import path from 'path';
import matter from 'gray-matter';
import { remark } from 'remark';
import html from 'remark-html';

const documentsDirectory = 'db';

export function getSortedDocumentsData() {

    const fileNames = fs.readdirSync(documentsDirectory);

    const allDocumentsData = fileNames.map((fileName) => {
        // Remove ".md" from file name to get id
        const id = fileName.replace(/\.md$/, '');
    
        // Read markdown file as string
        const fullPath = path.join(documentsDirectory, fileName);
        const fileContents = fs.readFileSync(fullPath, 'utf8');
    
        // Use gray-matter to parse the post metadata section
        const matterResult = matter(fileContents);
    
        // Combine the data with the id
        return {
          id,
          ...matterResult.data,
        };
      });

      return allDocumentsData.sort(({ title: a }, { title: b }) => {
        if (a < b) {
          return 1;
        } else if (a > b) {
          return -1;
        } else {
          return 0;
        }
      });

    return [];
}

export function getAllDocumentIds(){

  const fileNames = fs.readdirSync(documentsDirectory);

  return fileNames.map((fileName) => {

    return {
      params: {
        id: fileName.replace(/\.md$/, ''),
      },
    };

  });
}

export async function getDocumentData(id){

  const fullPath = path.join(documentsDirectory, `${id}.md`);
  const fileContents = fs.readFileSync(fullPath, 'utf8');

  // Use gray-matter to parse the post metadata section
  const matterResult = matter(fileContents);
  const contentMd = matterResult.content;

  // Use remark to convert markdown into HTML string
  const processedContent = await remark()
    .use(html)
    .process(matterResult.content);
  const contentHtml = processedContent.toString();

  // Combine the data with the id
  return {
    id,
    contentMd,
    contentHtml,
    ...matterResult.data,
  };
}