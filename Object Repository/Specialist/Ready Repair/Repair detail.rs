<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Repair detail</name>
   <tag></tag>
   <elementGuidId>2de1e1b3-d010-4b03-b385-c99616a1c1c3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>${cookies}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url}module/cdg/main/specialist/inProgressDetail/${id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>272ff01e-f8cc-4b09-8bb8-369d1475fa9c</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cookiesspecialist</defaultValue>
      <description></description>
      <id>58d1f6f3-e721-4d77-8c97-4ee3c910ba8c</id>
      <masked>false</masked>
      <name>cookies</name>
   </variables>
   <variables>
      <defaultValue>'5d1ad2f3c7c95d12c14c0652'</defaultValue>
      <description></description>
      <id>3bda8009-96aa-4198-9e45-988f77e021b6</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import org.junit.Before

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

WS.delay('6')

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

println(response)

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())

//JsonSlurper slurper = new JsonSlurper()
//
//Map parsedJson = slurper.parseText(response.getResponseText())

WS.verifyElementPropertyValue(response, 'success', true)

WS.verifyElementPropertyValue(response, 'data.serviceProduct.status', 'repair')

WS.verifyElementPropertyValue(response, 'data', jsonResponse.data)

def variables = request.getVariables()

def id = variables.get('id')

WS.verifyElementPropertyValue(response, 'data._id.$oid', id )

//parsedJson.get('message')
//
//parsedJson.get(&quot;data&quot;).get(&quot;_id&quot;)
//
//parsedJson.get(&quot;data&quot;).get(&quot;customer&quot;)
//
//parsedJson.get(&quot;data&quot;).get(&quot;serviceProduct&quot;)
//
//parsedJson.get(&quot;data&quot;).get(&quot;case&quot;)

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
