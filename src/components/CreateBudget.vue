<template>
	<div id="blur">
  <div id="create-budget">
		<button @click="toggleBudget()" id="cancel" title="Cancelar"><svg-icon type="mdi" :path="mdiClose" /></button>
    <input id="budget-number" v-model="id" placeholder="0001-0000001"></input>
		<form id="budget-form">
			<div id="customer-data">
				<div>
					<label for="customer">Cliente</label>
					<VueSelect class="vue-select"
						id="customer"
						v-model="customer"
						:options="customers"
						placeholder="Nombre Apellido"/>
				</div>
				<div>
					<label for="vehicle">Vehiculo</label>
					<VueSelect class="vue-select"
						id="vehicle"
						v-model="vehicle"
						:options="domains"
						placeholder="ABC123"/>
				</div>
				<div>
					<label for="concept">Concepto</label>
					<VueSelect class="vue-select"
						id="concept"
						v-model="concept"
						:options="concepts" 
						placeholder="Reparación"/>
				</div>
				<div>
					<label for="kilometrage">Kilometraje</label>
					<input id="kilometrage" v-model="kilometrage" type="text" pattern="\d*">
				</div>
			</div>
      <div id="table">
				<label for="item">Item</label>
				<label for="price">Importe</label>
				<label for="cant">Cant.</label>
				<label for="tipo">Tipo</label>
				<label for="iva" title="%">IVA</label>
				<label for="save-item"/>
				<VueSelect class="vue-select"
					item="item"
					v-model="item"
					:options="inventory"
					placeholder="Buscar producto o servicio"
					@update:modelValue="fillDetail"
				/>
				<input id="price" v-model="price" placeholder="0.0"></input>
				<input id="cant" v-model="cant" placeholder="0"></input>
				<VueSelect class="vue-select"
					id="tipo"
					v-model="tipo"
					:options="[{label: 'Producto', value: 'product'}, {label: 'Servicio', value: 'service'}]"
					placeholder="Producto o Servicio"
				/>
				<input id="iva" v-model="iva" placeholder="21" pattern="\d*"></input>
				<button id="save-item" title="Añadir item" @click="addDetail" type="button"><svg-icon type="mdi" :path="mdiPlus" /></button> 
      </div>
		<div id="details">
			<p>Item</p>
			<p>Precio</p>
			<p>Cant</p>
			<p>Tipo</p>
			<p>Subtotal</p>
			<p>IVA</p>
			<p>Total</p>
		</div>
		<Detail
        v-for="(detail, index) in details"
        :key="index"
        :detail="detail"
        :index="index"
        :delDetail="delDetail"
      />
    </form>
		<button id="confirm" title="Crear Presupuesto" @click="createBudget" type="button"><svg-icon type="mdi" :path="mdiCheck"/></button>
  </div>
	</div>
</template>

<script>
import { ref, watch, onMounted } from 'vue';                
import { invoke } from '@tauri-apps/api/core';
import VueSelect from "vue3-select-component";

import Detail from "@components/Detail.vue";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiClose, mdiPlus, mdiCheck } from '@mdi/js';

const concepts = ref([{ label: 'Reparación', value: 'reparacion' },
		{ label: 'Service', value: 'service' }, { label: 'Mantenimiento', value: 'mantenimiento' },
		{ label: 'Revisión', value: 'revision' }, { label: 'Garantia', value: 'garantia' },
		{ label: 'Otros', value: 'otros' }]);
// Form fields
const customer = ref("");
const vehicle = ref("");
const concept = ref("");
const kilometrage = ref(0.0);
const details = ref([]);
// details fields
const id = ref();
const item = ref("");
const price = ref(0.0);
const cant = ref(0);
const tipo = ref("");
const iva = ref(0.0);
//values for selects
const customers = ref([])
const domains = ref([])
const inventory = ref([])

export default {
	name: 'CreateBudget',
	components: {
		SvgIcon,
		VueSelect,
		Detail,
	},
	methods: {
		toggleBudget() {
			const userConfirmed = confirm("¿Seguro de cerrar? Los cambios no se guardaran")
			if (!userConfirmed) {return 0}
			this.$emit('destroy');
		}
	},
	setup(props, { emit }) {
		// Get data for Select
		const updateVehicles = async() => {
			let res = await invoke('obtain_vehicles')
			for (let r in res) {
				domains.value.push({"label": res[r].domain, "value":res[r].domain})
			}
		}
		const updateCustomers = async() => {
			let res = await invoke('obtain_customers')
			for (let r in res) {
				customers.value.push({"label": res[r].name, "value":res[r].name})
			}
		}
		const updateInventory = async() => {
			let res = await invoke('obtain_items')
			for (let r in res) {
				inventory.value.push({"label": res[r].name, "value":res[r]})
			}
		}
		
		const fillDetail = (selected) => {
			console.log(selected)
			price.value = "";
			tipo.value = "";
			if (selected != undefined) {
				price.value = selected.price
				tipo.value = selected.tipo
			}
		}

		const createBudget = async() => {
			// Get actual date
			const today = new Date();
			const formattedDate = today.toLocaleDateString('en-GB');

			// Get total amount of the detail
			let amount = details.value.reduce((accumulator, currentValue) => {
        return accumulator + currentValue.total;
      }, 0);
			amount = parseFloat(amount)

			await invoke('create_budget', {
				id: id.value, date: formattedDate, customer: customer.value, vehicle: vehicle.value,
				concept: concept.value, kilometrage: parseFloat(kilometrage.value), total: amount,
				details: details.value
			})

			// Notify father 
			emit('refresh-budgets');
			emit('destroy');
		}

		// Check when change id of budget
    let timeout = null;
    const handleInputChange = () => {
      clearTimeout(timeout);

      timeout = setTimeout(() => {
        //TO-DO: check if id is available;
      }, 400);
    };
    watch(id, handleInputChange);

		// Detail functions
		const delDetail = (index) => {details.value.splice(index, 1);};

		const addDetail = () => {
			// Parse Values
			let stotal = price.value*cant.value;
			let ivaPrice = parseFloat(stotal*iva.value/100);
			let total = parseFloat(stotal+ivaPrice);

			details.value.push({
				id: id.value,
				item: item.value.name,
				price: parseFloat(price.value),
				cant: parseInt(cant.value),
				tipo: tipo.value,
				subtotal: stotal,
				iva: ivaPrice,
				total: total});
		};

		addEventListener('keydown', (e) => {
			if (e.keyCode == 27) {
				let cancelBtn = document.querySelector('#cancel')
				cancelBtn.click()
			}
		})
		onMounted(updateVehicles)
		onMounted(updateCustomers)
		onMounted(updateInventory)
		return {
			customers,
			domains,
			inventory,
			concepts,
			fillDetail,
			// Input vars
			id,
			customer,
			vehicle,
			concept,
			kilometrage,
			details,
			// Inside detail
			item, price, cant, tipo, iva,
			// Functions
			addDetail,
			delDetail,
			createBudget,
			// Icons
			mdiClose,	mdiPlus, mdiCheck
		};
	},
};
</script>

<style scoped>
/* --  Budget PopUp  -- */
#blur {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: #0005;
	z-index: 90;
}
#blur.hidden {
  display: none;
}
#create-budget {
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
  display: flex;
  background-color: #202020;
  border: 2px solid #444;
  border-radius: 9px;
	z-index: 100;
}
#create-budget > button {
  position: absolute;
  display: flex;
  cursor: pointer;
  height: 2rem;
  width: 2rem;
  border: 1px solid #999;
  outline: none;
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
#budget-number {
  margin: 0px;
  position: absolute;
  top: .6rem;
  right: .6rem;
}
#create-budget.hidden {
  display: none;
}
#budget-form {
	height: 78%;
  width: 90%;
  display: flex;
  flex-direction: column;
  align-items: center;
	overflow-y: scroll;
	overflow-x: scroll;	
}
#customer-data {
	width: 100%;
	display: grid;
	grid-template-columns: 40% 40%;
	column-gap: 20%;
}
input {
	background: #333;
	border: none;
	border-radius: .4rem;
	padding: 6px 0.5rem;
	font-size: 18px;
	font-weight: 400;
	color: #111;
}
input::placeholder {
	color: #52525b;
}
/* --  Details  -- */
#table {
	width: 98%;
	margin: 0;
	margin-top: 2rem;
	align-items: center;
	display: grid;
	gap: 10px;
	grid-template-columns: 17.4rem 5rem 3rem 12rem 3rem 1.8rem;
}
#price, #stotal {width: 4.2rem;}

#cant,
#iva
{width: 2rem;}

#item,
#tipo,
{width: 9rem;}

#save-item {
	display: flex;
	align-items: center;
	justify-content: center;
	cursor: pointer;
	height: 2rem;
	width: 2rem;
	background: #333;
	border: 1px solid #999;
	border-radius: .4rem;
	transition: background .2s;
}
#save-item:hover {
	background: #088;
}
#details {
	width: 98%;
	display: grid;
	gap: 10px;
	grid-template-columns: auto auto auto 110px 130px 85px auto auto;
	justify-content: start;
	align-items: center;
}
#details > * {
	text-decoration: underline;
	padding: 0 12px;
	margin-bottom: 8px;
}
/* --  Custom Select  -- */
:deep(.vue-select) {
  --vs-input-bg: #333;
  --vs-border: 1px solid #eee0;
  --vs-border-radius: .4rem;
}
</style>
