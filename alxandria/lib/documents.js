// Using local file system as database for now
import fs from 'fs';

import path from 'path';
import matter from 'gray-matter';
import { remark } from 'remark';
import html from 'remark-html';

const documentsDirectory = 'db';

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

// save document
export function saveDocumentData(id, contentMd) {

  const fullPath = path.join(documentsDirectory, `${id}.md`);
  console.log(fullPath)
  // fs.writeFileSync(fullPath, contentMd);
  return;
}

// create document

//get document