<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Product list</name>
   <tag></tag>
   <elementGuidId>7c5bc695-dc96-4085-95b6-d3e306a34618</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;function&quot;,
      &quot;value&quot;: &quot;datatable&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[draw]&quot;,
      &quot;value&quot;: &quot;1&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][0][data]&quot;,
      &quot;value&quot;: &quot;customer&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][0][name]&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][0][searchable]&quot;,
      &quot;value&quot;: &quot;true&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][0][orderable]&quot;,
      &quot;value&quot;: &quot;true&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][0][search][value]&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][0][search][regex]&quot;,
      &quot;value&quot;: &quot;false&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][1][data]&quot;,
      &quot;value&quot;: &quot;name&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][1][name]&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][1][searchable]&quot;,
      &quot;value&quot;: &quot;true&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][1][orderable]&quot;,
      &quot;value&quot;: &quot;true&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][1][search][value]&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][1][search][regex]&quot;,
      &quot;value&quot;: &quot;false&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][2][data]&quot;,
      &quot;value&quot;: &quot;warrantyStatus&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][2][name]&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][2][searchable]&quot;,
      &quot;value&quot;: &quot;true&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][2][orderable]&quot;,
      &quot;value&quot;: &quot;true&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][2][search][value]&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][2][search][regex]&quot;,
      &quot;value&quot;: &quot;false&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][3][data]&quot;,
      &quot;value&quot;: &quot;purchaseDate&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][3][name]&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][3][searchable]&quot;,
      &quot;value&quot;: &quot;true&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][3][orderable]&quot;,
      &quot;value&quot;: &quot;true&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][3][search][value]&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][3][search][regex]&quot;,
      &quot;value&quot;: &quot;false&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][4][data]&quot;,
      &quot;value&quot;: &quot;invoice&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][4][name]&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][4][searchable]&quot;,
      &quot;value&quot;: &quot;true&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][4][orderable]&quot;,
      &quot;value&quot;: &quot;true&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][4][search][value]&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][4][search][regex]&quot;,
      &quot;value&quot;: &quot;false&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][5][data]&quot;,
      &quot;value&quot;: &quot;actionBtn&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][5][name]&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][5][searchable]&quot;,
      &quot;value&quot;: &quot;true&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][5][orderable]&quot;,
      &quot;value&quot;: &quot;true&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][5][search][value]&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[columns][5][search][regex]&quot;,
      &quot;value&quot;: &quot;false&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[order][0][column]&quot;,
      &quot;value&quot;: &quot;0&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[order][0][dir]&quot;,
      &quot;value&quot;: &quot;asc&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[start]&quot;,
      &quot;value&quot;: &quot;0&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[length]&quot;,
      &quot;value&quot;: &quot;10&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[search][value]&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[search][regex]&quot;,
      &quot;value&quot;: &quot;false&quot;,
      &quot;type&quot;: &quot;Text&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>${cookies}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}module/cdg/admin/product/table/all</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>e2cfbbc5-23e6-4d90-a488-9c5be86ab723</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cookies</defaultValue>
      <description></description>
      <id>8fe7194c-aaf0-474c-9914-20af69031068</id>
      <masked>false</masked>
      <name>cookies</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())

println(jsonResponse.data != null)

WS.verifyElementPropertyValue(response, 'data', jsonResponse.data)

WS.verifyElementPropertyValue(response, 'success', true)

WS.verifyElementPropertyValue(response, 'message', '[]')
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
