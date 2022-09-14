-- CreateTable
CREATE TABLE "Document" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "title" TEXT NOT NULL,
    "version" INTEGER NOT NULL,
    "date" DATETIME NOT NULL,
    "markdown" TEXT NOT NULL
);
