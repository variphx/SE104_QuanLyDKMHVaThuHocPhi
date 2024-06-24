<script lang="ts">
  type Nganh = {
    id: string;
    ten: string;
  };

  export let nganhs: Nganh[];
</script>

<div class="table-container">
  <table class="table table-hover">
    <thead>
      <tr>
        <th> Mã ngành </th>
        <th> Tên ngành </th>
        <th> Xóa </th>
      </tr>
    </thead>
    <tbody>
      {#each nganhs as nganh}
        <tr>
          <td>
            {nganh.id}
          </td>
          <td>
            {nganh.ten}
          </td>
          <td> fdslfdj </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
