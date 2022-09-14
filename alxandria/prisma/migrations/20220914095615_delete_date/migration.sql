/*
  Warnings:

  - You are about to drop the column `date` on the `Document` table. All the data in the column will be lost.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Document" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "title" TEXT NOT NULL,
    "version" INTEGER NOT NULL,
    "markdown" TEXT NOT NULL
);
INSERT INTO "new_Document" ("id", "markdown", "title", "version") SELECT "id", "markdown", "title", "version" FROM "Document";
DROP TABLE "Document";
ALTER TABLE "new_Document" RENAME TO "Document";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
