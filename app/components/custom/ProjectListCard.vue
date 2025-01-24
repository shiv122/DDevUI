<template>
  <Card>
    <CardContent class="p-0">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead v-for="(header, index) in headers" :key="index">
              <span v-if="header.srOnly" class="sr-only">{{
                header.label
              }}</span>
              <span v-else>{{ header.label }}</span>
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow v-for="(row, rowIndex) in tableData" :key="rowIndex">
            <TableCell v-if="row.image" class="hidden sm:table-cell">
              <img
                :alt="row.image.alt || 'Product image'"
                class="aspect-square rounded-md object-cover"
                :height="64"
                :width="64"
                :src="row.image.src"
              />
            </TableCell>
            <TableCell
              v-for="(cell, cellIndex) in row.cells"
              :key="cellIndex"
              :class="cell.class || ''"
            >
              <template v-if="cell.type === 'badge'">
                <Badge :variant="cell.variant || 'outline'">{{
                  cell.content
                }}</Badge>
              </template>
              <template v-else>
                {{ cell.content }}
              </template>
            </TableCell>
            <TableCell>
              <DropdownMenu>
                <DropdownMenuTrigger as-child>
                  <Button aria-haspopup="true" size="icon" variant="ghost">
                    <MoreHorizontal class="h-4 w-4" />
                    <span class="sr-only">Toggle menu</span>
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  <DropdownMenuItem
                    v-for="(action, actionIndex) in actions"
                    :key="actionIndex"
                    @click="action.onClick(row)"
                  >
                    {{ action.label }}
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </CardContent>
  </Card>
</template>

<script setup>
import { MoreHorizontal } from "lucide-vue-next";
defineProps({
  headers: {
    type: Array,
    required: true,
    default: () => [
      { label: "img", srOnly: true },
      { label: "Name" },
      { label: "Status" },
      { label: "Price" },
      { label: "Total Sales" },
      { label: "Created at" },
      { label: "Actions", srOnly: true },
    ],
  },
  tableData: {
    type: Array,
    required: true,
    default: () => [],
  },
  actions: {
    type: Array,
    required: true,
    default: () => [
      { label: "Edit", onClick: (row) => null },
      { label: "Delete", onClick: (row) => null },
    ],
  },
});
</script>
