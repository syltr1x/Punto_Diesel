<template>
  <div id="blur">
		<div id="create-item">
			<button @click="toggleItem()" id="cancel" title="Cancelar"><svg-icon type="mdi" :path="mdiClose" /></button>
    <form id="item-form">
				<div class="cols">
					<label>Nombre *</label>
					<input v-model="name" placeholder="Cambio de Aceite" />
				</div>
				<div class="cols">
					<label>ID</label>
					<input v-model="idn" :disabled="idDisabled" type="text" placeholder="0000 or F0000">
				</div>
				<div class="cols">
					<label>Precio *</label>
					<input v-model="price" placeholder="0.000,00" />
				</div>
				<div class="cols">
					<label>Tipo de Item *</label>
					<VueSelect class="vue-select"
						v-model="tipo"
						:options="[
							{ label: 'Producto', value: 'product' },
							{ label: 'Servicio', value: 'service' },
						]" placeholder="Producto"/>
				</div>
				<div :class="{cols:true, hidden: tipo != 'product'}">
					<label for="brand">Marca</label>
					<input v-model="brand" id="brand" placeholder="Denso" />
				</div>
				<div :class="{cols:true, hidden: tipo != 'product'}">
					<label>Modelo</label>
					<input v-model="model" type="text" placeholder="Inyector Common Rail">
				</div>
				<div :class="{cols:true, hidden: tipo != 'product'}">
					<label>Proveedor</label>
					<input v-model="supplier" type="text" placeholder="Rosario Filtros">
				</div>
				<div :class="{cols:true, hidden: tipo != 'product'}">
					<label>Stock</label>
					<input v-model="stock" type="text" placeholder="99">
				</div>
    </form>
		<button id="confirm" title="Añadir Item" @click="addItem" type="button"><svg-icon type="mdi" :path="mdiCheck" /></button>
		</div>
		</div>
</template>

<script>
import { ref } from 'vue';                
import { invoke } from "@tauri-apps/api/core";
import VueSelect from "vue3-select-component";
import SvgIcon from "@jamescoyle/vue-icon";
import { mdiClose, mdiCheck } from "@mdi/js"

export default {
	props: {
		data: {
			type: Object,
			required: false,
		}
	},
	name: 'CreateItem',
	components: {
		VueSelect,
		SvgIcon,
	},
	setup(props, { emit }) {
		const idDisabled = ref(false);
		const name = ref(null);
		const idn = ref(null);
		const price = ref(null);
		const tipo = ref(null);
		const brand = ref(null);
		const model = ref(null);
		const supplier = ref(null);
		const stock = ref(null);

		if (props.data != undefined) {
			idDisabled.value = true
			name.value = props.data.name
			idn.value = props.data.id
			price.value = props.data.price
			tipo.value = props.data.tipo
			if (props.data.tipo == "Producto") {
				brand.value = props.data.brand
				model.value = props.data.model
				supplier.value = props.data.supplier
				stock.value = props.data.stock
			}
		}

		const toggleItem = () => {
			if (name.value == null && price.value == null) {emit('destroy'); return 0}
			const userConfirmed = confirm("¿Seguro de cerrar? Los cambios no se guardaran")
			if (!userConfirmed) {return 0}
			emit('clear-item')
			emit('destroy');
		}
		const addItem = async() => {
			let log = await invoke('create_item', {'id':idn.value, 'name':name.value,
			'price':parseFloat(price.value), 'tipo':tipo.value, 'manufacturer':brand.value,
			'supplier':supplier.value, 'model':model.value, 'stock':parseInt(stock.value)})
			alert(log)
			emit('refresh-items')
			emit('destroy')
		}
		return {
			idDisabled,
			// Input vars
			name,
			idn,
			price,
			tipo,
			brand,
			model,
			supplier,
			stock,
			// Functions
			toggleItem,
			addItem,
			// Icons
			mdiClose, mdiCheck
		};
	},
};
</script>

<style scoped>
/* --  Create budget box  -- */
#blur {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: #0005;
	z-index: 90;
}
#create-item {
  display: flex;
  position: absolute;
  align-items: center;
  justify-content: center;
  width: 90%;
  height: 60%;
  min-width: 315px;
  overflow-y: hidden;
  overflow-x: hidden;
  color: #ddd;
  top: 20%;
  left: 5%;
  background-color: #202020;
  border: 2px solid #668076;
  border-radius: 9px;
	z-index: 100;
}
#create-item > button {
  position: absolute;
  display: flex;
  cursor: pointer;
  height: 30px;
  width: 30px;
  outline: none;
  border: 1px solid #999;
  border-radius: .4rem;
	background: #333;
	color: white;
	align-items: center;
	justify-content: center;
	transition: background .2s;
}
#cancel {
	top: .6rem;
	left: .6rem;
}
#confirm {
	bottom: .6rem;
	right: .6rem;
}
#cancel:hover {
	background: #543c3c;
}
#confirm:hover {
	background: #434f3b;
}
#item-form {
	height: 90%;
	margin: auto;
  width: 100%;
	display: grid;
	grid-template-columns: 50% 50%;
	grid-template-rows: repeat(4, 1fr);
	justify-items: center;
	overflow-y: scroll;
	overflow-x: scroll;
}
#item-form input {
	background: #333;
	border: none;
	border-radius: .4rem;
	padding: 6px 0.5rem;
	font-size: 18px;
	font-weight: 400;
}
#item-form input::placeholder {
	color: #52525b;
}
#item-form input:focus {
	outline: 2px solid #777;
}
.cols {
	display: flex;
	width: 40%;
	flex-direction: column;
	align-items: space-between;
}
.cols > label {
  margin: 0 0 .4rem 0;
}
.cols.hidden {
	display: none;
}
/* --  Custom Select  -- */
:deep(.vue-select) {
  --vs-input-bg: #333;
  --vs-border: 1px solid #eee0;
  --vs-border-radius: .4rem;
	--vs-input-outline: #777;
}
</style>
