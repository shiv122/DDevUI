<template>
  <div class="flex flex-1 flex-col gap-4 lg:gap-6">
    <div class="flex items-center justify-between">
      <h1 class="text-lg font-semibold md:text-2xl">Projects</h1>
      <NuxtLink to="/add-project">
        <Button size="sm" class="h-7 gap-1">
          <PlusCircle class="h-3.5 w-3.5" />
          <span class="sr-only whitespace-normal w-fit sm:not-sr-only sm:whitespace-nowrap">
            Add Project
          </span>
        </Button>
      </NuxtLink>
    </div>
    <ProjectListCard :headers="headers" :tableData="tableData" :actions="actions" />
  </div>
</template>

<script setup>
import { ListFilter, File, PlusCircle } from "lucide-vue-next";
// import Database from "@tauri-apps/plugin-sql";

onMounted(async () => {
  // const db = await Database.load("sqlite:ddevui.db");
  // const projects = await db.select("SELECT * FROM projects");
  // console.log(projects);
});

const headers = [
  { label: "img", srOnly: true },
  { label: "Name" },
  { label: "Status" },
  { label: "Price" },
  { label: "Total Sales" },
  { label: "Created at" },
  { label: "Actions", srOnly: true },
];

const tableData = [
  {
    image: { src: "/logo.png", alt: "Product image" },
    cells: [
      { content: "Laser Lemonade Machine", class: "font-medium" },
      {
        content: "Online",
        type: "badge",
        variant: "outline",
      },
      { content: "$499.99", class: "hidden md:table-cell" },
      { content: "25", class: "hidden md:table-cell" },
      { content: "2023-07-12 10:42 AM", class: "hidden md:table-cell" },
    ],
  },
  // Add more rows here
];

const actions = [
  {
    label: "Edit",
    onClick: (row) => console.log(`Editing ${row.cells[0].content}`),
  },
  {
    label: "Delete",
    onClick: (row) => console.log(`Deleting ${row.cells[0].content}`),
  },
];
</script>
